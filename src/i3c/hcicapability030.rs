#[doc = "Register `HCICAPABILITY030` reader"]
pub type R = crate::R<Hcicapability030Spec>;
#[doc = "Register `HCICAPABILITY030` writer"]
pub type W = crate::W<Hcicapability030Spec>;
#[doc = "Field `REGDATTABLEOFFSET` reader - REG_DAT_TABLE_OFFSET"]
pub type RegdattableoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGDATTABLESIZE` reader - REG_DAT_TABLE_SIZE"]
pub type RegdattablesizeR = crate::FieldReader;
#[doc = "Field `REGDATENTRYSIZE` reader - REG_DAT_ENTRY_SIZE"]
pub type RegdatentrysizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - REG_DAT_TABLE_OFFSET"]
    #[inline(always)]
    pub fn regdattableoffset(&self) -> RegdattableoffsetR {
        RegdattableoffsetR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:18 - REG_DAT_TABLE_SIZE"]
    #[inline(always)]
    pub fn regdattablesize(&self) -> RegdattablesizeR {
        RegdattablesizeR::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bits 28:31 - REG_DAT_ENTRY_SIZE"]
    #[inline(always)]
    pub fn regdatentrysize(&self) -> RegdatentrysizeR {
        RegdatentrysizeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "DAT\\_SECTION\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability030Spec;
impl crate::RegisterSpec for Hcicapability030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability030::R`](R) reader structure"]
impl crate::Readable for Hcicapability030Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability030::W`](W) writer structure"]
impl crate::Writable for Hcicapability030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY030 to value 0x0007_f100"]
impl crate::Resettable for Hcicapability030Spec {
    const RESET_VALUE: u32 = 0x0007_f100;
}
