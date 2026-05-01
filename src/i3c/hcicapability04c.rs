#[doc = "Register `HCICAPABILITY04C` reader"]
pub type R = crate::R<Hcicapability04cSpec>;
#[doc = "Register `HCICAPABILITY04C` writer"]
pub type W = crate::W<Hcicapability04cSpec>;
#[doc = "Field `REGICCSUPPORT` reader - REG_ICC_SUPPORT"]
pub type RegiccsupportR = crate::BitReader;
#[doc = "Field `REGMIPICMDSSUPPORTED` reader - REG_MIPI_CMDS_SUPPORTED"]
pub type RegmipicmdssupportedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - REG_ICC_SUPPORT"]
    #[inline(always)]
    pub fn regiccsupport(&self) -> RegiccsupportR {
        RegiccsupportR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15 - REG_MIPI_CMDS_SUPPORTED"]
    #[inline(always)]
    pub fn regmipicmdssupported(&self) -> RegmipicmdssupportedR {
        RegmipicmdssupportedR::new(((self.bits >> 1) & 0x7fff) as u16)
    }
}
impl W {}
#[doc = "INT\\_CTRL\\_CMDS\\_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability04cSpec;
impl crate::RegisterSpec for Hcicapability04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability04c::R`](R) reader structure"]
impl crate::Readable for Hcicapability04cSpec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability04c::W`](W) writer structure"]
impl crate::Writable for Hcicapability04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY04C to value 0x3f"]
impl crate::Resettable for Hcicapability04cSpec {
    const RESET_VALUE: u32 = 0x3f;
}
