#[doc = "Register `SCUE10` reader"]
pub type R = crate::R<Scue10Spec>;
#[doc = "Register `SCUE10` writer"]
pub type W = crate::W<Scue10Spec>;
#[doc = "Field `SCUREGLOCK210` reader - SCU_REG_LOCK_210"]
pub type Scureglock210R = crate::BitReader;
#[doc = "Field `SCUREGLOCK210` writer - SCU_REG_LOCK_210"]
pub type Scureglock210W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK214` reader - SCU_REG_LOCK_214"]
pub type Scureglock214R = crate::BitReader;
#[doc = "Field `SCUREGLOCK214` writer - SCU_REG_LOCK_214"]
pub type Scureglock214W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK218` reader - SCU_REG_LOCK_218"]
pub type Scureglock218R = crate::BitReader;
#[doc = "Field `SCUREGLOCK218` writer - SCU_REG_LOCK_218"]
pub type Scureglock218W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK21C` reader - SCU_REG_LOCK_21C"]
pub type Scureglock21cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK21C` writer - SCU_REG_LOCK_21C"]
pub type Scureglock21cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUREGLOCK230` reader - SCU_REG_LOCK_230"]
pub type Scureglock230R = crate::BitReader;
#[doc = "Field `SCUREGLOCK230` writer - SCU_REG_LOCK_230"]
pub type Scureglock230W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK234` reader - SCU_REG_LOCK_234"]
pub type Scureglock234R = crate::BitReader;
#[doc = "Field `SCUREGLOCK234` writer - SCU_REG_LOCK_234"]
pub type Scureglock234W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK238` reader - SCU_REG_LOCK_238"]
pub type Scureglock238R = crate::BitReader;
#[doc = "Field `SCUREGLOCK238` writer - SCU_REG_LOCK_238"]
pub type Scureglock238W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK23C` reader - SCU_REG_LOCK_23C"]
pub type Scureglock23cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK23C` writer - SCU_REG_LOCK_23C"]
pub type Scureglock23cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGLOCK250` reader - SCU_REG_LOCK_250"]
pub type Scureglock250R = crate::BitReader;
#[doc = "Field `SCUREGLOCK250` writer - SCU_REG_LOCK_250"]
pub type Scureglock250W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK254` reader - SCU_REG_LOCK_254"]
pub type Scureglock254R = crate::BitReader;
#[doc = "Field `SCUREGLOCK254` writer - SCU_REG_LOCK_254"]
pub type Scureglock254W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK258` reader - SCU_REG_LOCK_258"]
pub type Scureglock258R = crate::BitReader;
#[doc = "Field `SCUREGLOCK258` writer - SCU_REG_LOCK_258"]
pub type Scureglock258W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK25C` reader - SCU_REG_LOCK_25C"]
pub type Scureglock25cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK25C` writer - SCU_REG_LOCK_25C"]
pub type Scureglock25cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK270` reader - SCU_REG_LOCK_270"]
pub type Scureglock270R = crate::BitReader;
#[doc = "Field `SCUREGLOCK270` writer - SCU_REG_LOCK_270"]
pub type Scureglock270W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK274` reader - SCU_REG_LOCK_274"]
pub type Scureglock274R = crate::BitReader;
#[doc = "Field `SCUREGLOCK274` writer - SCU_REG_LOCK_274"]
pub type Scureglock274W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK278` reader - SCU_REG_LOCK_278"]
pub type Scureglock278R = crate::BitReader;
#[doc = "Field `SCUREGLOCK278` writer - SCU_REG_LOCK_278"]
pub type Scureglock278W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK27C` reader - SCU_REG_LOCK_27C"]
pub type Scureglock27cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK27C` writer - SCU_REG_LOCK_27C"]
pub type Scureglock27cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - SCU_REG_LOCK_210"]
    #[inline(always)]
    pub fn scureglock210(&self) -> Scureglock210R {
        Scureglock210R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_214"]
    #[inline(always)]
    pub fn scureglock214(&self) -> Scureglock214R {
        Scureglock214R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_218"]
    #[inline(always)]
    pub fn scureglock218(&self) -> Scureglock218R {
        Scureglock218R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_21C"]
    #[inline(always)]
    pub fn scureglock21c(&self) -> Scureglock21cR {
        Scureglock21cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_LOCK_230"]
    #[inline(always)]
    pub fn scureglock230(&self) -> Scureglock230R {
        Scureglock230R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_LOCK_234"]
    #[inline(always)]
    pub fn scureglock234(&self) -> Scureglock234R {
        Scureglock234R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_LOCK_238"]
    #[inline(always)]
    pub fn scureglock238(&self) -> Scureglock238R {
        Scureglock238R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_LOCK_23C"]
    #[inline(always)]
    pub fn scureglock23c(&self) -> Scureglock23cR {
        Scureglock23cR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - SCU_REG_LOCK_250"]
    #[inline(always)]
    pub fn scureglock250(&self) -> Scureglock250R {
        Scureglock250R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_REG_LOCK_254"]
    #[inline(always)]
    pub fn scureglock254(&self) -> Scureglock254R {
        Scureglock254R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SCU_REG_LOCK_258"]
    #[inline(always)]
    pub fn scureglock258(&self) -> Scureglock258R {
        Scureglock258R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SCU_REG_LOCK_25C"]
    #[inline(always)]
    pub fn scureglock25c(&self) -> Scureglock25cR {
        Scureglock25cR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - SCU_REG_LOCK_270"]
    #[inline(always)]
    pub fn scureglock270(&self) -> Scureglock270R {
        Scureglock270R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCU_REG_LOCK_274"]
    #[inline(always)]
    pub fn scureglock274(&self) -> Scureglock274R {
        Scureglock274R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SCU_REG_LOCK_278"]
    #[inline(always)]
    pub fn scureglock278(&self) -> Scureglock278R {
        Scureglock278R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_REG_LOCK_27C"]
    #[inline(always)]
    pub fn scureglock27c(&self) -> Scureglock27cR {
        Scureglock27cR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - SCU_REG_LOCK_210"]
    #[inline(always)]
    pub fn scureglock210(&mut self) -> Scureglock210W<Scue10Spec> {
        Scureglock210W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_214"]
    #[inline(always)]
    pub fn scureglock214(&mut self) -> Scureglock214W<Scue10Spec> {
        Scureglock214W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_218"]
    #[inline(always)]
    pub fn scureglock218(&mut self) -> Scureglock218W<Scue10Spec> {
        Scureglock218W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_21C"]
    #[inline(always)]
    pub fn scureglock21c(&mut self) -> Scureglock21cW<Scue10Spec> {
        Scureglock21cW::new(self, 7)
    }
    #[doc = "Bit 12 - SCU_REG_LOCK_230"]
    #[inline(always)]
    pub fn scureglock230(&mut self) -> Scureglock230W<Scue10Spec> {
        Scureglock230W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_LOCK_234"]
    #[inline(always)]
    pub fn scureglock234(&mut self) -> Scureglock234W<Scue10Spec> {
        Scureglock234W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_LOCK_238"]
    #[inline(always)]
    pub fn scureglock238(&mut self) -> Scureglock238W<Scue10Spec> {
        Scureglock238W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_LOCK_23C"]
    #[inline(always)]
    pub fn scureglock23c(&mut self) -> Scureglock23cW<Scue10Spec> {
        Scureglock23cW::new(self, 15)
    }
    #[doc = "Bit 20 - SCU_REG_LOCK_250"]
    #[inline(always)]
    pub fn scureglock250(&mut self) -> Scureglock250W<Scue10Spec> {
        Scureglock250W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_REG_LOCK_254"]
    #[inline(always)]
    pub fn scureglock254(&mut self) -> Scureglock254W<Scue10Spec> {
        Scureglock254W::new(self, 21)
    }
    #[doc = "Bit 22 - SCU_REG_LOCK_258"]
    #[inline(always)]
    pub fn scureglock258(&mut self) -> Scureglock258W<Scue10Spec> {
        Scureglock258W::new(self, 22)
    }
    #[doc = "Bit 23 - SCU_REG_LOCK_25C"]
    #[inline(always)]
    pub fn scureglock25c(&mut self) -> Scureglock25cW<Scue10Spec> {
        Scureglock25cW::new(self, 23)
    }
    #[doc = "Bit 28 - SCU_REG_LOCK_270"]
    #[inline(always)]
    pub fn scureglock270(&mut self) -> Scureglock270W<Scue10Spec> {
        Scureglock270W::new(self, 28)
    }
    #[doc = "Bit 29 - SCU_REG_LOCK_274"]
    #[inline(always)]
    pub fn scureglock274(&mut self) -> Scureglock274W<Scue10Spec> {
        Scureglock274W::new(self, 29)
    }
    #[doc = "Bit 30 - SCU_REG_LOCK_278"]
    #[inline(always)]
    pub fn scureglock278(&mut self) -> Scureglock278W<Scue10Spec> {
        Scureglock278W::new(self, 30)
    }
    #[doc = "Bit 31 - SCU_REG_LOCK_27C"]
    #[inline(always)]
    pub fn scureglock27c(&mut self) -> Scureglock27cW<Scue10Spec> {
        Scureglock27cW::new(self, 31)
    }
}
#[doc = "Write Protection 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue10Spec;
impl crate::RegisterSpec for Scue10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue10::R`](R) reader structure"]
impl crate::Readable for Scue10Spec {}
#[doc = "`write(|w| ..)` method takes [`scue10::W`](W) writer structure"]
impl crate::Writable for Scue10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE10 to value 0"]
impl crate::Resettable for Scue10Spec {}
