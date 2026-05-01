#[doc = "Register `I3CPHYCTRLREG004` reader"]
pub type R = crate::R<I3cphyctrlreg004Spec>;
#[doc = "Register `I3CPHYCTRLREG004` writer"]
pub type W = crate::W<I3cphyctrlreg004Spec>;
#[doc = "Field `REGSWSDAPULLUPEN` reader - REG_SW_SDA_PULLUP_EN"]
pub type RegswsdapullupenR = crate::FieldReader;
#[doc = "Field `REGSWSDAPULLUPEN` writer - REG_SW_SDA_PULLUP_EN"]
pub type RegswsdapullupenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REGSWSDAOE` reader - REG_SW_SDA_OE"]
pub type RegswsdaoeR = crate::BitReader;
#[doc = "Field `REGSWSDAOE` writer - REG_SW_SDA_OE"]
pub type RegswsdaoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSWSDAO` reader - REG_SW_SDA_O"]
pub type RegswsdaoR = crate::BitReader;
#[doc = "Field `REGSWSDAO` writer - REG_SW_SDA_O"]
pub type RegswsdaoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSWSDAI` reader - REG_SW_SDA_I"]
pub type RegswsdaiR = crate::BitReader;
#[doc = "Field `REGSWSDAI` writer - REG_SW_SDA_I"]
pub type RegswsdaiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGSWSCLPULLUPEN` reader - REG_SW_SCL_PULLUP_EN"]
pub type RegswsclpullupenR = crate::FieldReader;
#[doc = "Field `REGSWSCLPULLUPEN` writer - REG_SW_SCL_PULLUP_EN"]
pub type RegswsclpullupenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REGSWSCLOE` reader - REG_SW_SCL_OE"]
pub type RegswscloeR = crate::BitReader;
#[doc = "Field `REGSWSCLOE` writer - REG_SW_SCL_OE"]
pub type RegswscloeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSWSCLO` reader - REG_SW_SCL_O"]
pub type RegswscloR = crate::BitReader;
#[doc = "Field `REGSWSCLO` writer - REG_SW_SCL_O"]
pub type RegswscloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSWSCLI` reader - REG_SW_SCL_I"]
pub type RegswscliR = crate::BitReader;
#[doc = "Field `REGSWSCLI` writer - REG_SW_SCL_I"]
pub type RegswscliW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSWENABLE` reader - REG_SW_ENABLE"]
pub type RegswenableR = crate::FieldReader;
#[doc = "Field `REGSWENABLE` writer - REG_SW_ENABLE"]
pub type RegswenableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - REG_SW_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regswsdapullupen(&self) -> RegswsdapullupenR {
        RegswsdapullupenR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - REG_SW_SDA_OE"]
    #[inline(always)]
    pub fn regswsdaoe(&self) -> RegswsdaoeR {
        RegswsdaoeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REG_SW_SDA_O"]
    #[inline(always)]
    pub fn regswsdao(&self) -> RegswsdaoR {
        RegswsdaoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_SW_SDA_I"]
    #[inline(always)]
    pub fn regswsdai(&self) -> RegswsdaiR {
        RegswsdaiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - REG_SW_SCL_PULLUP_EN"]
    #[inline(always)]
    pub fn regswsclpullupen(&self) -> RegswsclpullupenR {
        RegswsclpullupenR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - REG_SW_SCL_OE"]
    #[inline(always)]
    pub fn regswscloe(&self) -> RegswscloeR {
        RegswscloeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REG_SW_SCL_O"]
    #[inline(always)]
    pub fn regswsclo(&self) -> RegswscloR {
        RegswscloR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REG_SW_SCL_I"]
    #[inline(always)]
    pub fn regswscli(&self) -> RegswscliR {
        RegswscliR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 24:31 - REG_SW_ENABLE"]
    #[inline(always)]
    pub fn regswenable(&self) -> RegswenableR {
        RegswenableR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_SW_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regswsdapullupen(&mut self) -> RegswsdapullupenW<I3cphyctrlreg004Spec> {
        RegswsdapullupenW::new(self, 0)
    }
    #[doc = "Bit 3 - REG_SW_SDA_OE"]
    #[inline(always)]
    pub fn regswsdaoe(&mut self) -> RegswsdaoeW<I3cphyctrlreg004Spec> {
        RegswsdaoeW::new(self, 3)
    }
    #[doc = "Bit 4 - REG_SW_SDA_O"]
    #[inline(always)]
    pub fn regswsdao(&mut self) -> RegswsdaoW<I3cphyctrlreg004Spec> {
        RegswsdaoW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_SW_SDA_I"]
    #[inline(always)]
    pub fn regswsdai(&mut self) -> RegswsdaiW<I3cphyctrlreg004Spec> {
        RegswsdaiW::new(self, 5)
    }
    #[doc = "Bits 8:10 - REG_SW_SCL_PULLUP_EN"]
    #[inline(always)]
    pub fn regswsclpullupen(&mut self) -> RegswsclpullupenW<I3cphyctrlreg004Spec> {
        RegswsclpullupenW::new(self, 8)
    }
    #[doc = "Bit 11 - REG_SW_SCL_OE"]
    #[inline(always)]
    pub fn regswscloe(&mut self) -> RegswscloeW<I3cphyctrlreg004Spec> {
        RegswscloeW::new(self, 11)
    }
    #[doc = "Bit 12 - REG_SW_SCL_O"]
    #[inline(always)]
    pub fn regswsclo(&mut self) -> RegswscloW<I3cphyctrlreg004Spec> {
        RegswscloW::new(self, 12)
    }
    #[doc = "Bit 13 - REG_SW_SCL_I"]
    #[inline(always)]
    pub fn regswscli(&mut self) -> RegswscliW<I3cphyctrlreg004Spec> {
        RegswscliW::new(self, 13)
    }
    #[doc = "Bits 24:31 - REG_SW_ENABLE"]
    #[inline(always)]
    pub fn regswenable(&mut self) -> RegswenableW<I3cphyctrlreg004Spec> {
        RegswenableW::new(self, 24)
    }
}
#[doc = "SW\\_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg004Spec;
impl crate::RegisterSpec for I3cphyctrlreg004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg004::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg004Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg004::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG004 to value 0x3737"]
impl crate::Resettable for I3cphyctrlreg004Spec {
    const RESET_VALUE: u32 = 0x3737;
}
