#[doc = "Register `HCICAPABILITY034` reader"]
pub type R = crate::R<Hcicapability034Spec>;
#[doc = "Register `HCICAPABILITY034` writer"]
pub type W = crate::W<Hcicapability034Spec>;
#[doc = "Field `REGDCTTABLEOFFSET` reader - REG_DCT_TABLE_OFFSET"]
pub type RegdcttableoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGDCTTABLESIZE` reader - REG_DCT_TABLE_SIZE"]
pub type RegdcttablesizeR = crate::FieldReader;
#[doc = "Field `REGDCTTABLEINDEX` reader - REG_DCT_TABLE_INDEX"]
pub type RegdcttableindexR = crate::FieldReader;
#[doc = "Field `REGDCTTABLEINDEX` writer - REG_DCT_TABLE_INDEX"]
pub type RegdcttableindexW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REGDCTENTRYSIZE` reader - REG_DCT_ENTRY_SIZE"]
pub type RegdctentrysizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - REG_DCT_TABLE_OFFSET"]
    #[inline(always)]
    pub fn regdcttableoffset(&self) -> RegdcttableoffsetR {
        RegdcttableoffsetR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:18 - REG_DCT_TABLE_SIZE"]
    #[inline(always)]
    pub fn regdcttablesize(&self) -> RegdcttablesizeR {
        RegdcttablesizeR::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bits 19:23 - REG_DCT_TABLE_INDEX"]
    #[inline(always)]
    pub fn regdcttableindex(&self) -> RegdcttableindexR {
        RegdcttableindexR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - REG_DCT_ENTRY_SIZE"]
    #[inline(always)]
    pub fn regdctentrysize(&self) -> RegdctentrysizeR {
        RegdctentrysizeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 19:23 - REG_DCT_TABLE_INDEX"]
    #[inline(always)]
    pub fn regdcttableindex(&mut self) -> RegdcttableindexW<Hcicapability034Spec> {
        RegdcttableindexW::new(self, 19)
    }
}
#[doc = "DCT\\_SECTION\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability034Spec;
impl crate::RegisterSpec for Hcicapability034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability034::R`](R) reader structure"]
impl crate::Readable for Hcicapability034Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability034::W`](W) writer structure"]
impl crate::Writable for Hcicapability034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY034 to value 0x1500"]
impl crate::Resettable for Hcicapability034Spec {
    const RESET_VALUE: u32 = 0x1500;
}
