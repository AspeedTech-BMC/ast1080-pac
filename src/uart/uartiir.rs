#[doc = "Register `UARTIIR` reader"]
pub type R = crate::R<UartiirSpec>;
#[doc = "Register `UARTIIR` writer"]
pub type W = crate::W<UartiirSpec>;
#[doc = "Field `IndicatesThatAnINTIsPendingWhenItsLogic0` reader - Indicates that an interrupt is pending when it's logic \"0\"."]
pub type IndicatesThatAnIntisPendingWhenItsLogic0R = crate::BitReader;
#[doc = "Field `INTDecodingTable` reader - Interrupt Decoding Table"]
pub type IntdecodingTableR = crate::FieldReader;
#[doc = "Field `Reserved1` reader - Reserved (0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `FIFOEnbldBits` reader - FIFO-Enabled Bits"]
pub type FifoenbldBitsR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Indicates that an interrupt is pending when it's logic \"0\"."]
    #[inline(always)]
    pub fn indicates_that_an_intis_pending_when_its_logic0(
        &self,
    ) -> IndicatesThatAnIntisPendingWhenItsLogic0R {
        IndicatesThatAnIntisPendingWhenItsLogic0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt Decoding Table"]
    #[inline(always)]
    pub fn intdecoding_table(&self) -> IntdecodingTableR {
        IntdecodingTableR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - FIFO-Enabled Bits"]
    #[inline(always)]
    pub fn fifoenbld_bits(&self) -> FifoenbldBitsR {
        FifoenbldBitsR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {}
#[doc = "Interrupt Identity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartiir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartiir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartiirSpec;
impl crate::RegisterSpec for UartiirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartiir::R`](R) reader structure"]
impl crate::Readable for UartiirSpec {}
#[doc = "`write(|w| ..)` method takes [`uartiir::W`](W) writer structure"]
impl crate::Writable for UartiirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTIIR to value 0x01"]
impl crate::Resettable for UartiirSpec {
    const RESET_VALUE: u32 = 0x01;
}
