#[doc = "Register `VIC060` reader"]
pub type R = crate::R<Vic060Spec>;
#[doc = "Register `VIC060` writer"]
pub type W = crate::W<Vic060Spec>;
#[doc = "Field `VICHBDEBOUNCE` reader - VIC_HB_DEBOUNCE"]
pub type VichbdebounceR = crate::FieldReader;
#[doc = "Field `VICHBDEBOUNCE` writer - VIC_HB_DEBOUNCE"]
pub type VichbdebounceW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VICHBSW` reader - VIC_HB_SW"]
pub type VichbswR = crate::BitReader;
#[doc = "Field `VICHBSW` writer - VIC_HB_SW"]
pub type VichbswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VICHBSWCNT` reader - VIC_HB_SW_CNT"]
pub type VichbswcntR = crate::FieldReader<u16>;
#[doc = "Field `VICHBSWCNT` writer - VIC_HB_SW_CNT"]
pub type VichbswcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `VICHBCM4ACTI` reader - VIC_HB_CM4ACT_I"]
pub type Vichbcm4actiR = crate::FieldReader;
#[doc = "Field `VICHBCM4ACTI` writer - VIC_HB_CM4ACT_I"]
pub type Vichbcm4actiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - VIC_HB_DEBOUNCE"]
    #[inline(always)]
    pub fn vichbdebounce(&self) -> VichbdebounceR {
        VichbdebounceR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - VIC_HB_SW"]
    #[inline(always)]
    pub fn vichbsw(&self) -> VichbswR {
        VichbswR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:14 - VIC_HB_SW_CNT"]
    #[inline(always)]
    pub fn vichbswcnt(&self) -> VichbswcntR {
        VichbswcntR::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - VIC_HB_CM4ACT_I"]
    #[inline(always)]
    pub fn vichbcm4acti(&self) -> Vichbcm4actiR {
        Vichbcm4actiR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - VIC_HB_DEBOUNCE"]
    #[inline(always)]
    pub fn vichbdebounce(&mut self) -> VichbdebounceW<Vic060Spec> {
        VichbdebounceW::new(self, 0)
    }
    #[doc = "Bit 4 - VIC_HB_SW"]
    #[inline(always)]
    pub fn vichbsw(&mut self) -> VichbswW<Vic060Spec> {
        VichbswW::new(self, 4)
    }
    #[doc = "Bits 5:14 - VIC_HB_SW_CNT"]
    #[inline(always)]
    pub fn vichbswcnt(&mut self) -> VichbswcntW<Vic060Spec> {
        VichbswcntW::new(self, 5)
    }
    #[doc = "Bits 28:31 - VIC_HB_CM4ACT_I"]
    #[inline(always)]
    pub fn vichbcm4acti(&mut self) -> Vichbcm4actiW<Vic060Spec> {
        Vichbcm4actiW::new(self, 28)
    }
}
#[doc = "HeartBeat Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vic060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic060Spec;
impl crate::RegisterSpec for Vic060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic060::R`](R) reader structure"]
impl crate::Readable for Vic060Spec {}
#[doc = "`write(|w| ..)` method takes [`vic060::W`](W) writer structure"]
impl crate::Writable for Vic060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC060 to value 0x1000_0321"]
impl crate::Resettable for Vic060Spec {
    const RESET_VALUE: u32 = 0x1000_0321;
}
