﻿namespace Platform::Data::Doublets::Decorators
{
    template <typename ...> class LinksUsagesValidator;
    template <typename TLink> class LinksUsagesValidator<TLink> : public LinksDecoratorBase<TLink>
    {
        public: LinksUsagesValidator(ILinks<TLink> &links) : LinksDecoratorBase(links) { }

        public: TLink Update(CList auto&&restrictions, CList auto&&substitution) override
        {
            auto links = _links;
            links.EnsureNoUsages(restrictions[_constants.IndexPart]);
            return links.Update(restrictions, substitution);
        }

        public: void Delete(CList auto&&restrictions) override
        {
            auto link = restrictions[_constants.IndexPart];
            auto links = _links;
            links.EnsureNoUsages(link);
            links.Delete(link);
        }
    };
}
