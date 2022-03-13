﻿namespace Platform::Data::Doublets::Decorators
{
    template <typename ...> class LinksUsagesValidator;
    template <typename TLink> class LinksUsagesValidator<TLink> : public LinksDecoratorBase<TFacade, TDecorated>
    {
        public: LinksUsagesValidator(ILinks<TLink> &storage) : LinksDecoratorBase(storage) { }

        public: TLink Update(CList auto&&restrictions, CList auto&&substitution) override
        {
            auto storage = this->decorated();
            storage.EnsureNoUsages(restrictions[_constants.IndexPart]);
            return storage.Update(restrictions, substitution);
        }

        public: void Delete(CList auto&&restrictions) override
        {
            auto link = restrictions[_constants.IndexPart];
            auto storage = this->decorated();
            storage.EnsureNoUsages(link);
            storage.Delete(link);
        }
    };
}
