#[doc = "Register `SCUE28` reader"]
pub type R = crate::R<Scue28Spec>;
#[doc = "Register `SCUE28` writer"]
pub type W = crate::W<Scue28Spec>;
#[doc = "Field `SCUREGLOCK500` reader - SCU_REG_LOCK_500"]
pub type Scureglock500R = crate::BitReader;
#[doc = "Field `SCUREGLOCK500` writer - SCU_REG_LOCK_500"]
pub type Scureglock500W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK504` reader - SCU_REG_LOCK_504"]
pub type Scureglock504R = crate::BitReader;
#[doc = "Field `SCUREGLOCK504` writer - SCU_REG_LOCK_504"]
pub type Scureglock504W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK508` reader - SCU_REG_LOCK_508"]
pub type Scureglock508R = crate::BitReader;
#[doc = "Field `SCUREGLOCK508` writer - SCU_REG_LOCK_508"]
pub type Scureglock508W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK50C` reader - SCU_REG_LOCK_50C"]
pub type Scureglock50cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK50C` writer - SCU_REG_LOCK_50C"]
pub type Scureglock50cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK510` reader - SCU_REG_LOCK_510"]
pub type Scureglock510R = crate::BitReader;
#[doc = "Field `SCUREGLOCK510` writer - SCU_REG_LOCK_510"]
pub type Scureglock510W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK514` reader - SCU_REG_LOCK_514"]
pub type Scureglock514R = crate::BitReader;
#[doc = "Field `SCUREGLOCK514` writer - SCU_REG_LOCK_514"]
pub type Scureglock514W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK518` reader - SCU_REG_LOCK_518"]
pub type Scureglock518R = crate::BitReader;
#[doc = "Field `SCUREGLOCK518` writer - SCU_REG_LOCK_518"]
pub type Scureglock518W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK51C` reader - SCU_REG_LOCK_51C"]
pub type Scureglock51cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK51C` writer - SCU_REG_LOCK_51C"]
pub type Scureglock51cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK520` reader - SCU_REG_LOCK_520"]
pub type Scureglock520R = crate::BitReader;
#[doc = "Field `SCUREGLOCK520` writer - SCU_REG_LOCK_520"]
pub type Scureglock520W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGLOCK540` reader - SCU_REG_LOCK_540"]
pub type Scureglock540R = crate::BitReader;
#[doc = "Field `SCUREGLOCK540` writer - SCU_REG_LOCK_540"]
pub type Scureglock540W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK544` reader - SCU_REG_LOCK_544"]
pub type Scureglock544R = crate::BitReader;
#[doc = "Field `SCUREGLOCK544` writer - SCU_REG_LOCK_544"]
pub type Scureglock544W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK548` reader - SCU_REG_LOCK_548"]
pub type Scureglock548R = crate::BitReader;
#[doc = "Field `SCUREGLOCK548` writer - SCU_REG_LOCK_548"]
pub type Scureglock548W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK54C` reader - SCU_REG_LOCK_54C"]
pub type Scureglock54cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK54C` writer - SCU_REG_LOCK_54C"]
pub type Scureglock54cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK57C` reader - SCU_REG_LOCK_57C"]
pub type Scureglock57cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK57C` writer - SCU_REG_LOCK_57C"]
pub type Scureglock57cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_LOCK_500"]
    #[inline(always)]
    pub fn scureglock500(&self) -> Scureglock500R {
        Scureglock500R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_504"]
    #[inline(always)]
    pub fn scureglock504(&self) -> Scureglock504R {
        Scureglock504R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_508"]
    #[inline(always)]
    pub fn scureglock508(&self) -> Scureglock508R {
        Scureglock508R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_50C"]
    #[inline(always)]
    pub fn scureglock50c(&self) -> Scureglock50cR {
        Scureglock50cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_510"]
    #[inline(always)]
    pub fn scureglock510(&self) -> Scureglock510R {
        Scureglock510R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_514"]
    #[inline(always)]
    pub fn scureglock514(&self) -> Scureglock514R {
        Scureglock514R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_518"]
    #[inline(always)]
    pub fn scureglock518(&self) -> Scureglock518R {
        Scureglock518R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_51C"]
    #[inline(always)]
    pub fn scureglock51c(&self) -> Scureglock51cR {
        Scureglock51cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_520"]
    #[inline(always)]
    pub fn scureglock520(&self) -> Scureglock520R {
        Scureglock520R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - SCU_REG_LOCK_540"]
    #[inline(always)]
    pub fn scureglock540(&self) -> Scureglock540R {
        Scureglock540R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_LOCK_544"]
    #[inline(always)]
    pub fn scureglock544(&self) -> Scureglock544R {
        Scureglock544R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_REG_LOCK_548"]
    #[inline(always)]
    pub fn scureglock548(&self) -> Scureglock548R {
        Scureglock548R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_REG_LOCK_54C"]
    #[inline(always)]
    pub fn scureglock54c(&self) -> Scureglock54cR {
        Scureglock54cR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_REG_LOCK_57C"]
    #[inline(always)]
    pub fn scureglock57c(&self) -> Scureglock57cR {
        Scureglock57cR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_LOCK_500"]
    #[inline(always)]
    pub fn scureglock500(&mut self) -> Scureglock500W<Scue28Spec> {
        Scureglock500W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_504"]
    #[inline(always)]
    pub fn scureglock504(&mut self) -> Scureglock504W<Scue28Spec> {
        Scureglock504W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_508"]
    #[inline(always)]
    pub fn scureglock508(&mut self) -> Scureglock508W<Scue28Spec> {
        Scureglock508W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_50C"]
    #[inline(always)]
    pub fn scureglock50c(&mut self) -> Scureglock50cW<Scue28Spec> {
        Scureglock50cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_510"]
    #[inline(always)]
    pub fn scureglock510(&mut self) -> Scureglock510W<Scue28Spec> {
        Scureglock510W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_514"]
    #[inline(always)]
    pub fn scureglock514(&mut self) -> Scureglock514W<Scue28Spec> {
        Scureglock514W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_518"]
    #[inline(always)]
    pub fn scureglock518(&mut self) -> Scureglock518W<Scue28Spec> {
        Scureglock518W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_51C"]
    #[inline(always)]
    pub fn scureglock51c(&mut self) -> Scureglock51cW<Scue28Spec> {
        Scureglock51cW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_520"]
    #[inline(always)]
    pub fn scureglock520(&mut self) -> Scureglock520W<Scue28Spec> {
        Scureglock520W::new(self, 8)
    }
    #[doc = "Bit 16 - SCU_REG_LOCK_540"]
    #[inline(always)]
    pub fn scureglock540(&mut self) -> Scureglock540W<Scue28Spec> {
        Scureglock540W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_LOCK_544"]
    #[inline(always)]
    pub fn scureglock544(&mut self) -> Scureglock544W<Scue28Spec> {
        Scureglock544W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_REG_LOCK_548"]
    #[inline(always)]
    pub fn scureglock548(&mut self) -> Scureglock548W<Scue28Spec> {
        Scureglock548W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_REG_LOCK_54C"]
    #[inline(always)]
    pub fn scureglock54c(&mut self) -> Scureglock54cW<Scue28Spec> {
        Scureglock54cW::new(self, 19)
    }
    #[doc = "Bit 31 - SCU_REG_LOCK_57C"]
    #[inline(always)]
    pub fn scureglock57c(&mut self) -> Scureglock57cW<Scue28Spec> {
        Scureglock57cW::new(self, 31)
    }
}
#[doc = "Write Protection 11 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue28Spec;
impl crate::RegisterSpec for Scue28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue28::R`](R) reader structure"]
impl crate::Readable for Scue28Spec {}
#[doc = "`write(|w| ..)` method takes [`scue28::W`](W) writer structure"]
impl crate::Writable for Scue28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE28 to value 0"]
impl crate::Resettable for Scue28Spec {}
