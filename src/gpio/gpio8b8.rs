#[doc = "Register `GPIO8B8` reader"]
pub type R = crate::R<Gpio8b8Spec>;
#[doc = "Register `GPIO8B8` writer"]
pub type W = crate::W<Gpio8b8Spec>;
#[doc = "Field `GPIO168WrPrivilegeOfMaster` reader - GPIO168 Write Privilege of Master"]
pub type Gpio168wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO168WrPrivilegeOfMaster` writer - GPIO168 Write Privilege of Master"]
pub type Gpio168wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO169WrPrivilegeOfMaster` reader - GPIO169 Write Privilege of Master"]
pub type Gpio169wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO169WrPrivilegeOfMaster` writer - GPIO169 Write Privilege of Master"]
pub type Gpio169wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO170WrPrivilegeOfMaster` reader - GPIO170 Write Privilege of Master"]
pub type Gpio170wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO170WrPrivilegeOfMaster` writer - GPIO170 Write Privilege of Master"]
pub type Gpio170wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO171WrPrivilegeOfMaster` reader - GPIO171 Write Privilege of Master"]
pub type Gpio171wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO171WrPrivilegeOfMaster` writer - GPIO171 Write Privilege of Master"]
pub type Gpio171wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO168 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio168wr_privilege_of_master(&self) -> Gpio168wrPrivilegeOfMasterR {
        Gpio168wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO169 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio169wr_privilege_of_master(&self) -> Gpio169wrPrivilegeOfMasterR {
        Gpio169wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO170 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio170wr_privilege_of_master(&self) -> Gpio170wrPrivilegeOfMasterR {
        Gpio170wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO171 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio171wr_privilege_of_master(&self) -> Gpio171wrPrivilegeOfMasterR {
        Gpio171wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO168 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio168wr_privilege_of_master(&mut self) -> Gpio168wrPrivilegeOfMasterW<Gpio8b8Spec> {
        Gpio168wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO169 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio169wr_privilege_of_master(&mut self) -> Gpio169wrPrivilegeOfMasterW<Gpio8b8Spec> {
        Gpio169wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO170 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio170wr_privilege_of_master(&mut self) -> Gpio170wrPrivilegeOfMasterW<Gpio8b8Spec> {
        Gpio170wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO171 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio171wr_privilege_of_master(&mut self) -> Gpio171wrPrivilegeOfMasterW<Gpio8b8Spec> {
        Gpio171wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#42\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio8b8Spec;
impl crate::RegisterSpec for Gpio8b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio8b8::R`](R) reader structure"]
impl crate::Readable for Gpio8b8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio8b8::W`](W) writer structure"]
impl crate::Writable for Gpio8b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO8B8 to value 0xffff_ffff"]
impl crate::Resettable for Gpio8b8Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
