using System;
using Xunit;
using Platform.Memory;
using Platform.Data.Doublets.Memory.Split.Specific;
using TLink = System.UInt32;

namespace Platform.Data.Doublets.Tests
{
    /// <summary>
    /// <para>
    /// Represents the split memory int 32 links tests.
    /// </para>
    /// <para></para>
    /// </summary>
    public unsafe static class SplitMemoryUInt32LinksTests
    {
        /// <summary>
        /// <para>
        /// Tests that crud test.
        /// </para>
        /// <para></para>
        /// </summary>
        [Fact]
        public static void CRUDTest()
        {
            Using(links => links.TestCRUDOperations());
        }

        /// <summary>
        /// <para>
        /// Tests that raw numbers crud test.
        /// </para>
        /// <para></para>
        /// </summary>
        [Fact]
        public static void RawNumbersCRUDTest()
        {
            UsingWithExternalReferences(links => links.TestRawNumbersCRUDOperations());
        }

        /// <summary>
        /// <para>
        /// Tests that multiple random creations and deletions test.
        /// </para>
        /// <para></para>
        /// </summary>
        [Fact]
        public static void MultipleRandomCreationsAndDeletionsTest()
        {
            Using(links => links.DecorateWithAutomaticUniquenessAndUsagesResolution().TestMultipleRandomCreationsAndDeletions(500));
        }

        /// <summary>
        /// <para>
        /// Usings the action.
        /// </para>
        /// <para></para>
        /// </summary>
        /// <param name="action">
        /// <para>The action.</para>
        /// <para></para>
        /// </param>
        private static void Using(Action<ILinks<TLink>> action)
        {
            using (var dataMemory = new HeapResizableDirectMemory())
            using (var indexMemory = new HeapResizableDirectMemory())
            using (var memory = new UInt32SplitMemoryLinks(dataMemory, indexMemory))
            {
                action(memory);
            }
        }

        /// <summary>
        /// <para>
        /// Usings the with external references using the specified action.
        /// </para>
        /// <para></para>
        /// </summary>
        /// <param name="action">
        /// <para>The action.</para>
        /// <para></para>
        /// </param>
        private static void UsingWithExternalReferences(Action<ILinks<TLink>> action)
        {
            var contants = new LinksConstants<TLink>(enableExternalReferencesSupport: true);
            using (var dataMemory = new HeapResizableDirectMemory())
            using (var indexMemory = new HeapResizableDirectMemory())
            using (var memory = new UInt32SplitMemoryLinks(dataMemory, indexMemory, UInt32SplitMemoryLinks.DefaultLinksSizeStep, contants))
            {
                action(memory);
            }
        }
    }
}
