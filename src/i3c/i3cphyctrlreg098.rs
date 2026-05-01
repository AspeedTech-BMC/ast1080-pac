#[doc = "Register `I3CPHYCTRLREG098` reader"]
pub type R = crate::R<I3cphyctrlreg098Spec>;
#[doc = "Register `I3CPHYCTRLREG098` writer"]
pub type W = crate::W<I3cphyctrlreg098Spec>;
#[doc = "Field `REGDDRSDAPULLUPEN` reader - REG_DDR_SDA_PULLUP_EN"]
pub type RegddrsdapullupenR = crate::FieldReader;
#[doc = "Field `REGDDRSDAPULLUPEN` writer - REG_DDR_SDA_PULLUP_EN"]
pub type RegddrsdapullupenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGDDRSCLPULLUPEN` reader - REG_DDR_SCL_PULLUP_EN"]
pub type RegddrsclpullupenR = crate::FieldReader;
#[doc = "Field `REGDDRSCLPULLUPEN` writer - REG_DDR_SCL_PULLUP_EN"]
pub type RegddrsclpullupenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGODSDAPULLUPEN` reader - REG_OD_SDA_PULLUP_EN"]
pub type RegodsdapullupenR = crate::FieldReader;
#[doc = "Field `REGODSDAPULLUPEN` writer - REG_OD_SDA_PULLUP_EN"]
pub type RegodsdapullupenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGODSCLPULLUPEN` reader - REG_OD_SCL_PULLUP_EN"]
pub type RegodsclpullupenR = crate::FieldReader;
#[doc = "Field `REGODSCLPULLUPEN` writer - REG_OD_SCL_PULLUP_EN"]
pub type RegodsclpullupenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGPPSDAPULLUPEN` reader - REG_PP_SDA_PULLUP_EN"]
pub type RegppsdapullupenR = crate::FieldReader;
#[doc = "Field `REGPPSDAPULLUPEN` writer - REG_PP_SDA_PULLUP_EN"]
pub type RegppsdapullupenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGPPSCLPULLUPEN` reader - REG_PP_SCL_PULLUP_EN"]
pub type RegppsclpullupenR = crate::FieldReader;
#[doc = "Field `REGPPSCLPULLUPEN` writer - REG_PP_SCL_PULLUP_EN"]
pub type RegppsclpullupenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_DDR_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regddrsdapullupen(&self) -> RegddrsdapullupenR {
        RegddrsdapullupenR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_DDR_SCL_PULLUP_EN"]
    #[inline(always)]
    pub fn regddrsclpullupen(&self) -> RegddrsclpullupenR {
        RegddrsclpullupenR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_OD_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regodsdapullupen(&self) -> RegodsdapullupenR {
        RegodsdapullupenR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_OD_SCL_PULLUP_EN"]
    #[inline(always)]
    pub fn regodsclpullupen(&self) -> RegodsclpullupenR {
        RegodsclpullupenR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_PP_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regppsdapullupen(&self) -> RegppsdapullupenR {
        RegppsdapullupenR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_PP_SCL_PULLUP_EN"]
    #[inline(always)]
    pub fn regppsclpullupen(&self) -> RegppsclpullupenR {
        RegppsclpullupenR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_DDR_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regddrsdapullupen(&mut self) -> RegddrsdapullupenW<I3cphyctrlreg098Spec> {
        RegddrsdapullupenW::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_DDR_SCL_PULLUP_EN"]
    #[inline(always)]
    pub fn regddrsclpullupen(&mut self) -> RegddrsclpullupenW<I3cphyctrlreg098Spec> {
        RegddrsclpullupenW::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_OD_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regodsdapullupen(&mut self) -> RegodsdapullupenW<I3cphyctrlreg098Spec> {
        RegodsdapullupenW::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_OD_SCL_PULLUP_EN"]
    #[inline(always)]
    pub fn regodsclpullupen(&mut self) -> RegodsclpullupenW<I3cphyctrlreg098Spec> {
        RegodsclpullupenW::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_PP_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regppsdapullupen(&mut self) -> RegppsdapullupenW<I3cphyctrlreg098Spec> {
        RegppsdapullupenW::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_PP_SCL_PULLUP_EN"]
    #[inline(always)]
    pub fn regppsclpullupen(&mut self) -> RegppsclpullupenW<I3cphyctrlreg098Spec> {
        RegppsclpullupenW::new(self, 20)
    }
}
#[doc = "CR\\_SCL\\_SDA\\_PULLUP\\_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg098Spec;
impl crate::RegisterSpec for I3cphyctrlreg098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg098::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg098Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg098::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG098 to value 0x0077_0077"]
impl crate::Resettable for I3cphyctrlreg098Spec {
    const RESET_VALUE: u32 = 0x0077_0077;
}
