#[doc = "Register `GPIO814` reader"]
pub type R = crate::R<Gpio814Spec>;
#[doc = "Register `GPIO814` writer"]
pub type W = crate::W<Gpio814Spec>;
#[doc = "Field `GPIO004WrPrivilegeOfMaster` reader - GPIO004 Write Privilege of Master"]
pub type Gpio004wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO004WrPrivilegeOfMaster` writer - GPIO004 Write Privilege of Master"]
pub type Gpio004wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO005WrPrivilegeOfMaster` reader - GPIO005 Write Privilege of Master"]
pub type Gpio005wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO005WrPrivilegeOfMaster` writer - GPIO005 Write Privilege of Master"]
pub type Gpio005wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO006WrPrivilegeOfMaster` reader - GPIO006 Write Privilege of Master"]
pub type Gpio006wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO006WrPrivilegeOfMaster` writer - GPIO006 Write Privilege of Master"]
pub type Gpio006wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO007WrPrivilegeOfMaster` reader - GPIO007 Write Privilege of Master"]
pub type Gpio007wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO007WrPrivilegeOfMaster` writer - GPIO007 Write Privilege of Master"]
pub type Gpio007wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO004 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio004wr_privilege_of_master(&self) -> Gpio004wrPrivilegeOfMasterR {
        Gpio004wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO005 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio005wr_privilege_of_master(&self) -> Gpio005wrPrivilegeOfMasterR {
        Gpio005wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO006 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio006wr_privilege_of_master(&self) -> Gpio006wrPrivilegeOfMasterR {
        Gpio006wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO007 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio007wr_privilege_of_master(&self) -> Gpio007wrPrivilegeOfMasterR {
        Gpio007wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO004 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio004wr_privilege_of_master(&mut self) -> Gpio004wrPrivilegeOfMasterW<Gpio814Spec> {
        Gpio004wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO005 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio005wr_privilege_of_master(&mut self) -> Gpio005wrPrivilegeOfMasterW<Gpio814Spec> {
        Gpio005wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO006 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio006wr_privilege_of_master(&mut self) -> Gpio006wrPrivilegeOfMasterW<Gpio814Spec> {
        Gpio006wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO007 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio007wr_privilege_of_master(&mut self) -> Gpio007wrPrivilegeOfMasterW<Gpio814Spec> {
        Gpio007wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio814::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio814::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio814Spec;
impl crate::RegisterSpec for Gpio814Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio814::R`](R) reader structure"]
impl crate::Readable for Gpio814Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio814::W`](W) writer structure"]
impl crate::Writable for Gpio814Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO814 to value 0xffff_ffff"]
impl crate::Resettable for Gpio814Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
