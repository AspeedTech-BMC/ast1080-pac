#[doc = "Register `SCU32C` reader"]
pub type R = crate::R<Scu32cSpec>;
#[doc = "Register `SCU32C` writer"]
pub type W = crate::W<Scu32cSpec>;
#[doc = "Field `SCUDIPLLRDYCLK` reader - SCU_DIPLL_RDY_CLK"]
pub type ScudipllrdyclkR = crate::BitReader;
#[doc = "Field `SCUDIPLLRDYGAIN` reader - SCU_DIPLL_RDY_GAIN"]
pub type ScudipllrdygainR = crate::BitReader;
#[doc = "Field `SCUDIPLLLOCKDET` reader - SCU_DIPLL_LOCK_DET"]
pub type ScudiplllockdetR = crate::FieldReader;
#[doc = "Field `SCUDIPLLLPFOUT` reader - SCU_DIPLL_LPF_OUT"]
pub type ScudiplllpfoutR = crate::FieldReader<u16>;
#[doc = "Field `SCUDIPLLFMOVER` reader - SCU_DIPLL_FM_OVER"]
pub type ScudipllfmoverR = crate::BitReader;
#[doc = "Field `SCUDIPLLFMUNDER` reader - SCU_DIPLL_FM_UNDER"]
pub type ScudipllfmunderR = crate::BitReader;
#[doc = "Field `SCUDIPLLFMRDY` reader - SCU_DIPLL_FM_RDY"]
pub type ScudipllfmrdyR = crate::BitReader;
#[doc = "Field `SCUDIPLLRDYTYPE1` reader - SCU_DIPLL_RDY_TYPE1"]
pub type Scudipllrdytype1R = crate::BitReader;
#[doc = "Field `SCUDIPLLFHRDY` reader - SCU_DIPLL_FH_RDY"]
pub type ScudipllfhrdyR = crate::BitReader;
#[doc = "Field `SCUDIPLLDBGOUT` reader - SCU_DIPLL_DBG_OUT"]
pub type ScudiplldbgoutR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - SCU_DIPLL_RDY_CLK"]
    #[inline(always)]
    pub fn scudipllrdyclk(&self) -> ScudipllrdyclkR {
        ScudipllrdyclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_DIPLL_RDY_GAIN"]
    #[inline(always)]
    pub fn scudipllrdygain(&self) -> ScudipllrdygainR {
        ScudipllrdygainR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - SCU_DIPLL_LOCK_DET"]
    #[inline(always)]
    pub fn scudiplllockdet(&self) -> ScudiplllockdetR {
        ScudiplllockdetR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:13 - SCU_DIPLL_LPF_OUT"]
    #[inline(always)]
    pub fn scudiplllpfout(&self) -> ScudiplllpfoutR {
        ScudiplllpfoutR::new(((self.bits >> 5) & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - SCU_DIPLL_FM_OVER"]
    #[inline(always)]
    pub fn scudipllfmover(&self) -> ScudipllfmoverR {
        ScudipllfmoverR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_DIPLL_FM_UNDER"]
    #[inline(always)]
    pub fn scudipllfmunder(&self) -> ScudipllfmunderR {
        ScudipllfmunderR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_DIPLL_FM_RDY"]
    #[inline(always)]
    pub fn scudipllfmrdy(&self) -> ScudipllfmrdyR {
        ScudipllfmrdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_DIPLL_RDY_TYPE1"]
    #[inline(always)]
    pub fn scudipllrdytype1(&self) -> Scudipllrdytype1R {
        Scudipllrdytype1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIPLL_FH_RDY"]
    #[inline(always)]
    pub fn scudipllfhrdy(&self) -> ScudipllfhrdyR {
        ScudipllfhrdyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:26 - SCU_DIPLL_DBG_OUT"]
    #[inline(always)]
    pub fn scudiplldbgout(&self) -> ScudiplldbgoutR {
        ScudiplldbgoutR::new(((self.bits >> 19) & 0xff) as u8)
    }
}
impl W {}
#[doc = "DIPLL Parameter Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`scu32c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu32c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu32cSpec;
impl crate::RegisterSpec for Scu32cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu32c::R`](R) reader structure"]
impl crate::Readable for Scu32cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu32c::W`](W) writer structure"]
impl crate::Writable for Scu32cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU32C to value 0"]
impl crate::Resettable for Scu32cSpec {}
