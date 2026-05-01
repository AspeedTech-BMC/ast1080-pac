#[doc = "Register `I3CCONTROL0BC` reader"]
pub type R = crate::R<I3ccontrol0bcSpec>;
#[doc = "Register `I3CCONTROL0BC` writer"]
pub type W = crate::W<I3ccontrol0bcSpec>;
#[doc = "Field `GROUPADDR0` reader - GROUP_ADDR_0"]
pub type Groupaddr0R = crate::FieldReader;
#[doc = "Field `GROUPADDR0VALID` reader - GROUP_ADDR_0_VALID"]
pub type Groupaddr0validR = crate::BitReader;
#[doc = "Field `GROUPADDR1` reader - GROUP_ADDR_1"]
pub type Groupaddr1R = crate::FieldReader;
#[doc = "Field `GROUPADDR1VALID` reader - GROUP_ADDR_1_VALID"]
pub type Groupaddr1validR = crate::BitReader;
#[doc = "Field `GROUPADDR2` reader - GROUP_ADDR_2"]
pub type Groupaddr2R = crate::FieldReader;
#[doc = "Field `GROUPADDR2VALID` reader - GROUP_ADDR_2_VALID"]
pub type Groupaddr2validR = crate::BitReader;
#[doc = "Field `GROUPADDR3` reader - GROUP_ADDR_3"]
pub type Groupaddr3R = crate::FieldReader;
#[doc = "Field `GROUPADDR3VALID` reader - GROUP_ADDR_3_VALID"]
pub type Groupaddr3validR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - GROUP_ADDR_0"]
    #[inline(always)]
    pub fn groupaddr0(&self) -> Groupaddr0R {
        Groupaddr0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - GROUP_ADDR_0_VALID"]
    #[inline(always)]
    pub fn groupaddr0valid(&self) -> Groupaddr0validR {
        Groupaddr0validR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - GROUP_ADDR_1"]
    #[inline(always)]
    pub fn groupaddr1(&self) -> Groupaddr1R {
        Groupaddr1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - GROUP_ADDR_1_VALID"]
    #[inline(always)]
    pub fn groupaddr1valid(&self) -> Groupaddr1validR {
        Groupaddr1validR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - GROUP_ADDR_2"]
    #[inline(always)]
    pub fn groupaddr2(&self) -> Groupaddr2R {
        Groupaddr2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - GROUP_ADDR_2_VALID"]
    #[inline(always)]
    pub fn groupaddr2valid(&self) -> Groupaddr2validR {
        Groupaddr2validR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - GROUP_ADDR_3"]
    #[inline(always)]
    pub fn groupaddr3(&self) -> Groupaddr3R {
        Groupaddr3R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - GROUP_ADDR_3_VALID"]
    #[inline(always)]
    pub fn groupaddr3valid(&self) -> Groupaddr3validR {
        Groupaddr3validR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "I3C\\_SLV\\_CTL\\_0BC\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0bcSpec;
impl crate::RegisterSpec for I3ccontrol0bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0bc::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0bcSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0bc::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0BC to value 0"]
impl crate::Resettable for I3ccontrol0bcSpec {}
